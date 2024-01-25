{
  description = "Awesome rsh flake";

  outputs = { self, nixpkgs }: {
		packages.aarch64-darwin.default = nixpkgs.legacyPackages.aarch64-darwin.hello;
		# packages.aarch64-darwin.hello = nixpkgs.legacyPackages.aarch64-darwin.hello; 

		# checks.aarch64-darwin."test" = derivation;
  };
}
