# Development Spec

## Goal
Padronizar o projeto para manter legibilidade, previsibilidade e evolução incremental, com **KISS, SOLID, DRY e YAGNI por padrão**.

## Mandatory Rules
1. **Arquitetura por domínio**
- `types/` concentra structs, enums e params structs.
- `constants/` concentra strings, limites e tuning constants.
- `app/`, `render/`, `wad/`, `framebuffer/`, `domain/` concentram comportamento.

2. **Funções com muitos parâmetros**
- Funções com mais de 2 parâmetros devem receber um params struct.
- Deve haver destructuring explícito no início da função.

3. **Sem magic numbers e strings espalhadas**
- Números de regra de negócio/renderização e textos fixos devem ir para `constants/`.
- Exceções: valores de teste localizados e literais estruturais curtos.

4. **Validações legíveis**
- Toda validação não trivial deve usar variáveis booleanas nomeadas.
- Evitar condicionais densas sem contexto semântico.

5. **Nomenclatura**
- Evitar variáveis de uma letra fora de escopo matemático curto.
- Preferir nomes de intenção: `ray_distance`, `segment_factor`, `clamped_distance`.

6. **Perfil de fluxo mais flat**
- Evitar código com excesso de indentação e blocos aninhados longos.
- Preferir early return, helper pequeno, `match` objetivo e composição que reduza profundidade visual.
- Quando houver escolha entre manter lógica inline ou extrair um passo claro para achatar o fluxo, preferir a versão mais flat.

7. **Testes organizados**
- Testes de módulo devem ficar em pastas de teste do módulo (`*/tests/`) ou em mod tests internos quando o caso for puramente local.
- Evitar espalhar testes em locais ambíguos.

8. **SOLID/KISS/DRY/YAGNI na prática**
- SRP: cada função faz uma única coisa clara.
- DRY: extrair duplicação quando houver repetição real.
- YAGNI: não criar abstrações sem demanda comprovada.
- KISS: preferir o design mais simples que atenda os requisitos.

## Code Style Conventions
- Params structs em `types/` usam sufixo `Params`.
- Helpers de mapeamento/normalização devem ser pequenos e puros quando possível.
- Comentários devem explicar "por quê" e não "o quê" óbvio.

## Current Compliance Actions
1. Projeto migrado para params structs centrais em `types/*` com destructuring explícito no body:
- `CastRayParams`
- `RaySegmentIntersectionParams`
- `RenderThingsParams`
- `SampleWallColorParams`
- `SampleSectorFlatColorParams`
- `PlaneWorldSampleParams`
- `SamplePalettePixelsParams`
- `ProceduralFlatColorParams`
- `ProceduralWallColorParams`
- `WallDetailParams`
- `BoolLabelParams`
- `RuntimeStatusTextParams`
- `ComposeTexturesParams`
- `BlitPatchParams`
- `LoadFlatsParams`
- `ReadNameParams`

2. Magic numbers de raycast e gameplay movidos para `constants/`.

3. Nomes de variáveis de interseção/render/framebuffer tornados semânticos.

4. Testes organizados por módulo em diretórios `tests/` (`app/tests`, `render/tests`, `wad/tests`).

5. Cadeias `else if` removidas dos trechos críticos de renderização/textura em favor de `match` e early-return.

6. Fluxos com muita indentação devem ser revisados para um perfil mais flat sempre que a extração não piorar coesão.

## Remaining Standardization Backlog
1. Revisar nomenclatura de campos de coordenada em tipos de domínio (`x/y`) para eventual migração para nomes mais explícitos (`world_x/world_y`) sem quebrar compatibilidade.
2. Consolidar checklist automatizado de conformidade da spec (lint/CI) para impedir regressões de padrão.

## Definition Of Done For New Changes
- Build e testes passam.
- Nenhum novo magic number sem justificativa.
- Nenhuma função nova com mais de 2 parâmetros sem params struct.
- Validações complexas com booleanos nomeados.
- Strings de UI e erros centralizadas.
- Fluxo de controle sem indentação excessiva quando houver alternativa mais flat viável.
