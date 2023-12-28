{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs,}:
    let
      pkgs = import nixpkgs {
        inherit system;
      };

      system = "x86_64-linux";
      app = "game";

      shellInputs = with pkgs; [
        rust-analyzer
      ];
      appNativeBuildInputs = with pkgs; [
        pkg-config
      ];
      appBuildInputs = appRuntimeInputs ++ (with pkgs; [
        udev alsaLib wayland
        vulkan-tools vulkan-headers vulkan-validation-layers
        libxkbcommon 
      ]);
      appRuntimeInputs = with pkgs; [
        vulkan-loader
        xorg.libXcursor xorg.libXi xorg.libXrandr
        wayland
        libxkbcommon 
        libGL
        libGLU
      ];
    in
    {
      devShells.${system}.${app} = pkgs.mkShell {
        nativeBuildInputs = appNativeBuildInputs;
        buildInputs = shellInputs ++ appBuildInputs;

        shellHook = ''
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath appRuntimeInputs}"
        '';
      };
      devShell.${system} = self.devShells.${system}.${app};

      packages.${system}.${app} = pkgs.rustPlatform.buildRustPackage {
        pname = app;
        version = "0.1.0";

        src = ./.;
        cargoSha256 = "sha256-lzs+8qAsBJ/ms/OppxnKfJChV9+xM0W/QRZGPn+9uv4=";

        nativeBuildInputs = appNativeBuildInputs;
        buildInputs = appBuildInputs;

        postInstall = ''
          cp -r assets $out/bin/
        '';
      };
      defaultPackage.${system} = self.packages.${system}.${app};

      apps.${system}.${app} = {
        type = "app";
        program = "${self.packages.${system}.${app}}/bin/${app}";
      };
      defaultApp.${system} = self.apps.${system}.${app};

      checks.${system}.build = self.packages.${system}.${app};
    };
}
