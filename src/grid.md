``` mermaid
    classDiagram
        %% Planos futuros:
        %%procedural wave collapse para a geração da grid baseadas em biomas e lugares comuns
        %%3 battle view (real, semi-tatic, tatic)
        %% -------------------
        %%componente para dizer oq tem em cima da celula, se ocupa a celula ou nao, se tapa a visão ou não, se é renderizado ou não, se tem algum efeito ou nao
        %% sobre as bordas, vincular a um sistema generico (talvez possa ser no proprio arquivo)
        %% Resources
        class GridXYZ {
            <<Resouce>>
            Esse resource me diz o máximo do x,y e z da grid.
            Esse resouce vai substituir as constantes.
            Se esse resource atualizar, a grid deve ser respawnada.
        }
        class GridBackup {
            <<Resource>>
            Esse resource vai guardar o StandartMaterial original da grid gerada e as configurações padrões dela.
        }
        class StandartMaterial {
            <<Resource>>
        }
        class Mesh {
            <<Resource>>
        }
        %% Components
        class PbrBundle {
            <<ComponentBundle>>
        }
        class Transform {
            <<Component>>
        }
        class PickableBundle {
            <<Component>>
        }
        class CellBundle {
            <<ComponentBundle>>
            Bundle que tem todos os componentes de todas as celulas possiveis
        }
        class BasicsAttributes {
            <<Component>>
            Se a celula é interagivel (definido pelo Type of Cell), ela tem esse componente
        }
        class CellAttributes {
            <<Component>>
            Componente de celulas interagiveis com caracteristicas somente de celulas.
        }
        class TypeOfCell {
            <<Component>>
            Dita o tipo da celula, mas não carrega nada consigo.
        }

        TypeOfCell <|-- BasicsAttributes
        TypeOfCell <|-- CellAttributes

        CellBundle o-- TypeOfCell
        CellBundle o-- PbrBundle
        CellBundle o-- PickableBundle

        PbrBundle o-- Mesh
        PbrBundle o-- StandartMaterial
        PbrBundle o-- Transform

        %%Systems
        class update_cell_type {
            <<System>>
            +GridBackup
            +GridXYZ
            -(StandartMaterial,Transform,TypeOfCell)
        }
        update_cell_type --> GridXYZ
        update_cell_type --> GridBackup

        %%StartupSystems
        class spawn_grid {
            <<StartupSystem>>
            ~Commands
            -Mesh
            -StandartMaterial
            -GridBackup
        }

        update_cell_type --> spawn_grid

        %% Main entity of the file
        class Grid {
            <<Entity>>
            Only one
        }
        Grid *-- PbrBundle
        class Cell {
            <<Entity>>
        }
        Cell *-- CellBundle
        Cell -- Grid
        Cell <-- update_cell_type
```