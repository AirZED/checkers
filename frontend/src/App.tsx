import React, { useState, useEffect } from "react";
// import { loadWasm } from "./utils/wasm";

const App = () => {
  const [piece, setPiece] = useState<number | null>(null);
  const [wasmInstance, setWasmInstance] = useState<WebAssembly.Instance | null>(
    null
  );

  useEffect(() => {
    fetch("../wasm/pkg/checkers_bg.wasm")
      .then((response) => response.arrayBuffer())
      .then((bytes) =>
        WebAssembly.instantiate(bytes, {
          env: {
            notify_piecemoved: (
              fromX: number,
              fromY: number,
              toX: number,
              toY: number
            ) => {
              console.log(
                `A piece moved from (${fromX}, ${fromY}) to (${toX}, ${toY})`
              );
            },
            notify_piececrowned: (x: number, y: number) => {
              console.log(`A piece was crowned at (${x}, ${y})`);
            },
          },
        })
      )
      .then((results) => {
        const instance = results.instance as WebAssembly.Instance & {
          exports: {
            get_current_turn: () => number;
            get_piece: (x: number, y: number) => number;
            move_piece: (
              fromX: number,
              fromY: number,
              toX: number,
              toY: number
            ) => number;
          };
        };
        console.log(
          `At start, current turn is ${instance.exports.get_current_turn()}`
        );
        let piece = instance.exports.get_piece(0, 7);
        console.log(`Piece at 0,7 is ${piece}`);
        let res = instance.exports.move_piece(0, 5, 1, 4); // Black
        console.log(`First move result: ${res}`);
        console.log(`Turn after move: ${instance.exports.get_current_turn()}`);
        let bad = instance.exports.move_piece(1, 4, 2, 3); // illegal move
        console.log(`Illegal move result:  ${bad}`);
        console.log(
          `Turn after illegal move: ${instance.exports.get_current_turn()}`
        );
      })
      .catch(console.error);
  }, []);

  return (
    <div>
      <h1>Checkers Game</h1>
      <p>Piece at (0, 0): {piece}</p>
    </div>
  );
};

export default App;
