<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import "../app.css";
  import Button from "@/components/ui/button/button.svelte";

  let name = "";
  let greetMsg = "";


  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    if(name === "") {
      greetMsg = "Please enter a name!";
      return;
    }
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>
 <div class=" text-blue-600 py-4">Lorem ipsum</div>


  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <Button type="submit">Greet</Button>
  </form>


  <p>{greetMsg}</p>
</main>

