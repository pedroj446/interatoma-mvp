# Comparativo técnico: protótipo `interatoma-mvp` vs merge do Cytoscape

## Escopo
Este documento compara o protótipo deste repositório com a funcionalidade de *merge* do Cytoscape em termos de eficiência e consumo de recursos, com foco no objetivo do projeto: rodar em máquinas de menor desempenho.

## Resumo executivo
- **Para poucos dados e fluxo bem específico (vírus-hospedeiro):** o protótipo tende a ser **mais leve** por usar pipeline simples (CSV -> `petgraph` -> exportação via API).  
- **Para cenários reais e grandes (múltiplas redes/tabelas, metadados, UI, undo, validações):** o *merge* do Cytoscape tende a ser **mais robusto e escalável em funcionalidades**, mas com **maior custo de memória/CPU** por ser uma plataforma completa de análise visual.

## Base da conclusão neste repositório
- O protótipo carrega dados CSV em memória (`Vec`) e monta grafo não-direcionado com `petgraph`, usando `HashMap`/`HashSet` para evitar nós/arestas duplicados.  
- O fluxo principal cria dois grafos separados e envia ambos para o Cytoscape via REST (`reqwest` bloqueante).  
- O dataset atual é muito pequeno (44 linhas totais nos CSVs versionados), então o desempenho observado localmente não representa carga real de proteoma completo.

## Eficiência (tempo) e consumo de recursos

### Protótipo `interatoma-mvp`
**Prós**
- Baixa complexidade de pipeline: parsing CSV + construção de grafo.
- Estrutura de deduplicação explícita reduz inserções redundantes.
- Sem interface gráfica própria, sem plugins, sem camadas extras de sessão.

**Contras**
- Leitura totalmente em memória (`Vec`) para proteínas e interações.
- Processo síncrono (blocking), sem streaming e sem paralelismo.
- Envia payload completo para Cytoscape; para redes grandes isso vira gargalo de serialização/transporte.

### Merge no Cytoscape (plataforma completa)
**Prós**
- Ferramenta madura para merge de redes/tabelas com recursos de curadoria e visualização.
- Melhor para operações interativas complexas e dados com muitos atributos.

**Contras**
- Overhead inerente de aplicação desktop Java + subsistemas de visualização.
- Maior consumo de RAM/CPU que um pipeline Rust minimalista, especialmente em computadores modestos.

## O que ficou bem feito no protótipo
1. **Modelagem simples e direta** (`Protein` com variante humana/viral, ID canônico para Cytoscape).
2. **Construção de grafo com deduplicação** de arestas via `HashSet` e índice de nós via `HashMap`.
3. **Integração automática com Cytoscape** (criação de rede + estilo visual por tipo de nó).
4. **Separação modular inicial** (`io`, `graph`, `model`, `cytoscape`, `analysis`).

## O que precisa melhorar para atingir o objetivo central
1. **Escalabilidade de entrada**
   - Trocar carregamento total em memória por processamento em fluxo (streaming/chunking).
   - Considerar formatos compactos (Parquet/Arrow) para datasets grandes.

2. **Medição real de desempenho**
   - Adicionar *benchmarks* reproduzíveis (tempo, pico de RAM, tamanho da rede).
   - Comparar contra baseline (merge no Cytoscape) com o mesmo dataset.

3. **Arquitetura para máquinas fracas**
   - Modo headless completo (sem depender do Cytoscape para etapas essenciais).
   - Estratégias de redução: filtros prévios, merge incremental, compressão de atributos.

4. **Confiabilidade e produto**
   - Melhor tratamento de erros e logs estruturados.
   - Testes automatizados (unitários + integração com amostras maiores).
   - CLI com parâmetros (entrada, saída, modo de merge, limites de memória).

## Veredito objetivo
- **Mais eficiente em recursos (no estado atual, para o problema reduzido):** `interatoma-mvp` tende a ser mais leve por design minimalista.
- **Mais completo para merge geral em bio-redes:** Cytoscape.
- **Para cumprir a proposta do projeto (merge de proteomas em hardware modesto):** o caminho certo é evoluir este protótipo para um motor de merge headless, medido por benchmark e com pipeline de dados escalável.
