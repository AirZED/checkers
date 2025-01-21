export const loadWasm = async () => {
  const wasm = await import("../../../pkg/checkers_bg.wasm");

  await wasm.default({
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
  });
  return wasm;
};
