import init, { WasmProvider } from "./pkg/papi_wasm_provider.js";
import { createClient } from "polkadot-api";
import { getWsProvider } from "@polkadot-api/ws-provider";

const status = document.getElementById("status");
const list = document.getElementById("blocks");
const t0 = performance.now() / 1000;
const elapsed = () => ((performance.now() / 1000) - t0).toFixed(1);

const params = new URLSearchParams(location.search);
const rpcUrl = params.get("rpc");

// Alice's well-known address (Polkadot SS58 prefix 0)
const ALICE = "15oF4uVJwmo4TdGW7VfQxNLavjCXviqxT9S1MgbjMNHr6Sp5";

const title = document.getElementById("title");
if (rpcUrl) {
  title.textContent = "PAPI + WebSocket RPC - Asset Hub";
} else {
  title.textContent = "PAPI + WASM Light Client - Asset Hub";
}

function formatDot(raw) {
  const n = typeof raw === "bigint" ? raw : BigInt(raw || 0);
  const whole = n / 10_000_000_000n;
  const frac = n % 10_000_000_000n;
  const fracStr = frac.toString().padStart(10, "0").replace(/0+$/, "") || "0";
  return `${whole.toLocaleString()}.${fracStr} DOT`;
}

function showBalance({ value: account }) {
  const section = document.getElementById("account");
  section.style.display = "";
  document.getElementById("acct-addr").textContent = ALICE;
  document.getElementById("acct-free").textContent = formatDot(account.data.free);
  document.getElementById("acct-reserved").textContent = formatDot(account.data.reserved);
  document.getElementById("acct-frozen").textContent = formatDot(account.data.frozen);
  document.getElementById("acct-nonce").textContent = String(account.nonce);
}

try {
  let provider;

  if (rpcUrl) {
    status.textContent = `Connecting to ${rpcUrl}... (${elapsed()}s)`;
    provider = getWsProvider(rpcUrl);
  } else {
    status.textContent = `Loading WASM module... (${elapsed()}s)`;
    await init();

    status.textContent = `Fetching chain specs... (${elapsed()}s)`;
    const [relaySpec, paraSpec] = await Promise.all([
      fetch("polkadot.json").then(r => r.text()),
      fetch("asset_hub.json").then(r => r.text()),
    ]);

    status.textContent = `Starting smoldot (WASM)... (${elapsed()}s)`;
    const wasmProvider = new WasmProvider(relaySpec, paraSpec);

    provider = (onMessage) => {
      wasmProvider.start((resp) => onMessage(JSON.parse(resp)));
      return {
        send: (msg) => wasmProvider.send(JSON.stringify(msg)),
        disconnect: () => wasmProvider.disconnect(),
      };
    };
  }

  status.textContent = `Creating PAPI client... (${elapsed()}s)`;
  const client = createClient(provider);
  const api = client.getUnsafeApi();
  const initTime = elapsed();

  status.textContent = `Ready in ${initTime}s, syncing...`;

  // Watch Alice's account balance (live updates through the same provider)
  api.query.System.Account.watchValue(ALICE).subscribe({
    next: showBalance,
    error: (e) => console.error("Balance watch error:", e),
  });

  let firstBlockTime = null;
  client.blocks$.subscribe((block) => {
    if (!firstBlockTime) firstBlockTime = elapsed();
    status.textContent = `#${block.number} (init ${initTime}s, first block ${firstBlockTime}s)`;

    window._lastBlockTime = performance.now();

    const li = document.createElement("li");
    const h = block.hash;
    li.textContent = `#${block.number} - ${h.slice(0, 6)}...${h.slice(-4)}`;
    list.prepend(li);
  });
} catch (e) {
  status.textContent = `Error: ${e.message}`;
  console.error(e);
}
