``` mermaid
    classDiagram
        %% Resources
        class CameraSpeed {
            <<Resouce>>
        }

        %% Components
        class Camera3dBundle {
            <<ComponentBundle>>
        }
        class Transform {
            <<Component>>
        }
        class Camera {
            <<Component>>
        }
        class PickingCameraBundle {
            <<Component>>
        }
        Camera3dBundle o-- Camera
        Camera3dBundle o-- Transform
        Camera3dBundle o-- PickingCameraBundle

        %%Systems
        class switch_camera {
            <<System>>
            +InputKeyCode
        }
        class move_camera {
            <<System>>
            +InputKeyCode
            -(Transform,Camera)
        }

        move_camera --> CameraSpeed

        %%StartupSystems
        class spawn_camera {
            <<StartupSystem>>
            ~Commands
        }

        move_camera --> spawn_camera
        switch_camera --> spawn_camera

        %% Main entity of the file
        class MainCamera {
            <<Entity>>
            Only one
        }
        class OrthoCamera {
            <<Entity>>
            Only one
        }

        MainCamera *-- Camera3dBundle
        MainCamera <-- move_camera
        MainCamera <-- switch_camera
        OrthoCamera *-- Camera3dBundle
        OrthoCamera <-- move_camera
        OrthoCamera <-- switch_camera

```