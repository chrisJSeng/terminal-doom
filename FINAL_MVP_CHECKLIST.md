# Final MVP Checklist - Doom Terminal

## Objective
Fechar uma versao jogavel e estavel do jogo com escopo MVP, mantendo o padrao definido em `DEVELOPMENT_SPEC.md`.

## Exit Criteria
- Build release sem erro: `cargo build --release`
- Testes passando: `cargo test`
- Smoke de execucao: `timeout 5 ./target/release/doom-terminal` (sem crash)
- Fluxo jogavel basico em `E1M1`: mover, rotacionar, renderizar, sair com `Esc`

## Execution Order

### 1. Gameplay Loop Closure
- Status: `done`
- Target files:
- `src/app/runtime/game_loop.rs`
- `src/app/runtime/movement.rs`
- `src/domain.rs`
- Acceptance:
- Movimento consistente com colisao em paredes
- Nao atravessar geometrias solidas
- Atualizacao por frame sem regressao de input
- Colisao de dominio filtrando linedefs solidos (nao colidir com linedef two-sided)
- Validation:
- `cargo test app::runtime::tests::try_move_player_with_collision_moves_when_no_collision`
- `cargo test app::runtime::tests::try_move_player_with_collision_rolls_back_on_collision`
- `cargo test domain::tests::collides_with_map_ignores_non_solid_linedef`

### 2. Raycast Visual Completeness
- Status: `done`
- Target files:
- `src/render/raycast.rs`
- `src/render/raycast/cast.rs`
- `src/render/raycast/shading.rs`
- `src/render/raycast/sprites.rs`
- `src/render/textures.rs`
- Acceptance:
- Paredes, chao e teto coerentes com setor/textura
- Sprites de `things` com clipping por profundidade funcionando
- Sem artefatos graves de depth ordering
- Render de sprites ordenado por profundidade (far-to-near) para reduzir sobreposicao incorreta
- Sprites com amostragem de textura/palette do WAD (com fallback seguro)
- Mapeamento semantico e deterministico de `thing_type` para grupo visual de textura
- Escala e offset vertical de sprite ajustados por `thing_type` para proporcoes mais legiveis
- Clipping lateral de sprite com mapeamento UV coberto por testes de borda da viewport
- Validation:
- `cargo test render::raycast::tests::cast_ray_computes_wall_u_from_hit_position`
- `cargo test render::raycast::tests::plane_world_sample_uses_world_coordinates`
- `cargo test render::textures::tests::test_distance_lighting_darkens_far`

### 3. WAD Data Robustness
- Status: `done`
- Target files:
- `src/wad/reader.rs`
- `src/wad/parser.rs`
- `src/constants/wad.rs`
- Acceptance:
- Carregamento robusto de lumps obrigatorios
- Validacao explicita do nome dos lumps obrigatorios do mapa por offset esperado
- Fallback seguro para lumps opcionais
- Erros de leitura com mensagens claras
- Header, diretorio e payload de lump truncados retornam erro claro em vez de EOF generico
- Composicao de textura continua com fallback seguro quando patch lump opcional esta ausente
- Validation:
- `cargo test wad::reader::tests::blit_patch_supports_tall_posts`
- `cargo test wad::reader::tests::load_flats_skips_absent_or_placeholder_names`

### 4. UX/HUD Final Pass
- Status: `done`
- Target files:
- `src/app/status.rs`
- `src/constants/ui.rs`
- `src/app/input/mapping.rs`
- `src/app/input/poll.rs`
- Acceptance:
- Status line util e consistente (mode/src/collision/hp/world)
- Teclas essenciais documentadas no HUD
- Sem texto hardcoded fora de `constants/`
- HUD alinhado com controles reais: `WS` mover, `AD` girar, `QE` zoom, `R/F` toggles, `Esc` sair
- Validation:
- `cargo test app::status::tests::build_status_text_contains_formatted_fields`
- `cargo test app::status::tests::backend_label_returns_expected_value`

### 5. Release Hardening
- Status: `done`
- Target files:
- `src/main.rs`
- `src/app/bootstrap.rs`
- `src/constants/*`
- Acceptance:
- Startup com diagnostico claro quando WAD ausente
- Sem panic inesperado no caminho normal
- Release executa por alguns segundos sem crash
- Validado binario release sem WAD: erro claro em stderr e exit code 1
- Validado smoke release com WAD: processo permanece estavel na janela do timeout
- Validation:
- `cargo build --release`
- `timeout 5 ./target/release/doom-terminal`

## Non-Functional Guardrails
- Manter padroes de params struct + destructuring para funcoes com mais de 2 parametros
- Evitar nesting excessivo; preferir fluxo flat com guard clauses
- Structs de producao ficam em `src/types/*`
- Strings e numeros de regra em `src/constants/*`

## Done Definition (Project)
- Todos os itens acima marcados como `done`
- `cargo test` verde
- `cargo build --release` verde
- Smoke manual jogavel executado e validado
- Nenhuma regressao introduzida nos testes existentes
