# Plano técnico executável (foco: merge de proteomas com baixo consumo)

## Objetivo
Evoluir o protótipo para um motor de merge que rode em hardware modesto, com medição de tempo e memória por execução.

## Entregas implementadas neste passo
- Modo **headless** (`--headless`) para executar sem Cytoscape e reduzir overhead.
- Modo **merge** (`--merge`) para gerar uma rede única (HTLV-1 + HTLV-2 + humano).
- Impressão de métricas básicas por execução: nós, arestas, grau médio e runtime total.

## Como executar
- Rede separada, sem Cytoscape (benchmark local):
  - `cargo run -- --headless`
- Rede já em merge:
  - `cargo run -- --headless --merge`
- Benchmark sintético rápido (sem Cytoscape):
  - `cargo run -- --headless --merge --bench-size=100000`
- Execução com auditoria em JSONL (opcional):
  - `cargo run -- --headless --merge --audit-log=audit.jsonl --log-level=info`
- Envio ao Cytoscape (se local rodando em `localhost:1234`):
  - `cargo run -- --merge`

## Fase 1 (curto prazo)
1. Criar benchmark reprodutível com datasets de 10k, 100k e 1M interações.
2. Coletar:
   - tempo total,
   - pico de memória,
   - tamanho do grafo final.
3. Definir meta inicial:
   - <= 2 GB RAM em 1M interações,
   - execução headless em máquina 8 GB RAM.

## Fase 2 (escala)
1. Substituir carregamento total (`Vec`) por pipeline streaming/chunked.
2. Adicionar deduplicação incremental em disco para datasets maiores.
3. Persistir saída em formato eficiente (Parquet/Arrow opcional).

## Fase 3 (produto)
1. CLI completa (`--input`, `--output`, `--max-memory`, `--threads`).
2. Testes automatizados (unitários + integração + benchmark CI).
3. Relatório automático comparando baseline Cytoscape x engine headless.
