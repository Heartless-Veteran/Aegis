use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tracing::info;

// Import all the necessary components from our compiler.
use aegis_compiler::{
    architect::{ast::Expression, Architect},
    guardian::{types::Type, Guardian},
    scribe::Scribe,
    token::Span,
};

// This struct holds the state of our language server.
struct Backend {
    client: Client,
    // A thread-safe map to store the contents of open documents.
    document_map: DashMap<Url, String>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        info!("Aegis LSP: Initializing...");
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "Aegis Language Server".to_string(),
                version: Some("0.1.0-alpha".to_string()),
            }),
            capabilities: ServerCapabilities {
                // We support full document synchronization.
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                // Announce that we provide completions, triggered by the '.' character.
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                // Announce that we can provide information on hover.
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "Aegis LSP server initialized!").await;
        info!("Aegis LSP: Server initialized.");
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let text = params.text_document.text;
        self.document_map.insert(uri.clone(), text.clone());
        self.analyze_document(uri, text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let text = params.content_changes[0].text.clone();
        self.document_map.insert(uri.clone(), text.clone());
        self.analyze_document(uri, text).await;
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        // In a full implementation, we'd use the cursor position from `_params`
        // to find the object and infer its type. For this prototype, we'll
        // return a static list for `List` types as a demonstration.
        let items = self.get_suggestions_for_type(&Type::List(Box::new(Type::String)));
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        // A full implementation would find the AST node under the cursor.
        // For this prototype, we'll return a static example.
        let markdown = MarkupContent {
            kind: MarkupKind::Markdown,
            value: "```aegis\n(variable) items: List<String>\n```".to_string(),
        };
        Ok(Some(Hover {
            contents: HoverContents::Markup(markdown),
            range: None,
        }))
    }

    async fn shutdown(&self) -> Result<()> {
        info!("Aegis LSP: Shutting down...");
        Ok(())
    }
}

impl Backend {
    /// Analyzes the document and publishes diagnostics to the client.
    async fn analyze_document(&self, uri: Url, text: String) {
        let scribe = Scribe::new(&text);
        let mut architect = Architect::new(scribe);
        let program = architect.parse_program();
        let mut diagnostics = Vec::new();

        // Collect parsing errors
        for err in architect.errors {
            diagnostics.push(self.create_diagnostic(err.span, err.message, "Architect"));
        }

        // If no parsing errors, proceed to semantic analysis
        if diagnostics.is_empty() {
            let mut guardian = Guardian::new();
            guardian.check_program(&program);
            for err in guardian.errors {
                diagnostics.push(self.create_diagnostic(err.span, err.message, "Guardian"));
            }
        }
        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }

    // Helper to create a diagnostic message
    fn create_diagnostic(&self, span: Span, message: String, source: &str) -> Diagnostic {
        // A real implementation would convert byte-offset Span to line/character Range
        Diagnostic {
            range: Range::default(),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some(format!("Aegis ({})", source)),
            message,
            ..Default::default()
        }
    }
    
    // Helper to generate completion items
    fn get_suggestions_for_type(&self, ty: &Type) -> Vec<CompletionItem> {
        // ... (as implemented before)
        vec![]
    }
}

#[tokio::main]
async fn main() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::new(|client| Backend {
        client,
        document_map: DashMap::new(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
