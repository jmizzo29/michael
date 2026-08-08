#!/usr/bin/env python3
"""Zero-dependency MICHAEL Clifford Torus + CA Hebbian training / inference harness."""

from __future__ import annotations

import math


class TorusEncoder:
    """Maps raw bytes to 3D Clifford Torus phase coordinates."""

    def encode(self, byte_val: int, prev_byte: int) -> tuple[float, float, float]:
        theta = (2.0 * math.pi * byte_val) / 256.0
        phi = (2.0 * math.pi * (byte_val ^ prev_byte)) / 256.0
        x = (2.0 + math.cos(phi)) * math.cos(theta)
        y = (2.0 + math.cos(phi)) * math.sin(theta)
        z = math.sin(phi)
        return (x, y, z)


class MichaelSubstrate:
    """The 2D Cellular Automata Manifold with local Hebbian Plasticity."""

    def __init__(self, size: int = 16):
        self.size = size
        self.grid = [[0.0 for _ in range(size)] for _ in range(size)]
        self.plasticity = [[1.0 for _ in range(size)] for _ in range(size)]

    def inject_phase(self, coords: tuple[float, float, float]):
        x, y, z = coords
        cx = int((x + 3.0) / 6.0 * (self.size - 1))
        cy = int((y + 3.0) / 6.0 * (self.size - 1))
        cx = max(0, min(self.size - 1, cx))
        cy = max(0, min(self.size - 1, cy))
        self.grid[cx][cy] += z

    def evolve(self, learning_rate: float = 0.05) -> float:
        new_grid = [[0.0 for _ in range(self.size)] for _ in range(self.size)]
        total_plasticity_change = 0.0

        for r in range(self.size):
            for c in range(self.size):
                up = self.grid[(r - 1) % self.size][c]
                down = self.grid[(r + 1) % self.size][c]
                left = self.grid[r][(c - 1) % self.size]
                right = self.grid[r][(c + 1) % self.size]

                local_field = (up + down + left + right) * self.plasticity[r][c]
                new_grid[r][c] = math.tanh(local_field) - 0.05 * math.sin(self.grid[r][c])

                if learning_rate > 0.0:
                    delta = learning_rate * (new_grid[r][c] * self.grid[r][c])
                    self.plasticity[r][c] = max(0.05, min(3.0, self.plasticity[r][c] + delta))
                    total_plasticity_change += abs(delta)

        self.grid = new_grid
        return total_plasticity_change

    def read_attractor_byte(self) -> int:
        energy = sum(sum(abs(val) for val in row) for row in self.grid)
        return int((energy * 100.0) % 256)


class MichaelTrainer:
    def __init__(self, size: int = 16):
        self.encoder = TorusEncoder()
        self.engine = MichaelSubstrate(size=size)

    def train(self, corpus: str, epochs: int = 10, lr: float = 0.05):
        print(f"=== STARTING TRAINING (Corpus Length: {len(corpus)} chars, Epochs: {epochs}) ===")
        bytes_data = list(corpus.encode("utf-8"))

        for epoch in range(1, epochs + 1):
            prev = 0
            epoch_loss = 0.0

            for b in bytes_data:
                coords = self.encoder.encode(b, prev)
                self.engine.inject_phase(coords)
                delta = self.engine.evolve(learning_rate=lr)
                epoch_loss += delta
                prev = b

            print(f"Epoch {epoch:2d}/{epochs} | Plasticity Adaptation Energy: {epoch_loss:.6f}")

        print("=== TRAINING COMPLETE: Substrate Plasticity Settled ===\n")

    def generate_bytes(self, prompt: str, gen_length: int = 20) -> list[int]:
        prompt_bytes = list(prompt.encode("utf-8"))
        prev = 0

        for b in prompt_bytes:
            coords = self.encoder.encode(b, prev)
            self.engine.inject_phase(coords)
            self.engine.evolve(learning_rate=0.0)
            prev = b

        out_bytes: list[int] = []
        for _ in range(gen_length):
            self.engine.evolve(learning_rate=0.0)
            next_b = self.engine.read_attractor_byte()
            out_bytes.append(next_b)

            coords = self.encoder.encode(next_b, prev)
            self.engine.inject_phase(coords)
            prev = next_b

        return out_bytes

    def generate(self, prompt: str, gen_length: int = 20) -> str:
        return bytes(self.generate_bytes(prompt, gen_length)).decode("utf-8", errors="ignore")


if __name__ == "__main__":
    michael = MichaelTrainer(size=16)
    training_data = "MICHAEL SYSTEM RESONANCE WAVE PATTERN LOGIC HEBBIAN ATTRACTOR " * 5
    michael.train(training_data, epochs=10, lr=0.02)

    test_prompt = "MICHAEL"
    out_bytes = michael.generate_bytes(test_prompt, gen_length=15)
    output = bytes(out_bytes).decode("utf-8", errors="replace")

    print(f"Input Prompt: '{test_prompt}'")
    print(f"Generated Output: {output!r}")
    print(f"Raw attractor bytes: {out_bytes}")
