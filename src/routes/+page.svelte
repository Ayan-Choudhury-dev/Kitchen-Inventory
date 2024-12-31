<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import "../app.css";
  import Button from "@/components/ui/button/button.svelte";
  import Input from "@/components/ui/input/input.svelte";

  let name = "";
  let greetMsg = "";
  let num1 = 0;
  let num2 = 0;
  let sum = 0;



  // async function greet(event: Event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   if(name === "") {
  //     greetMsg = "Please enter a name!";
  //     return;
  //   }
  //   greetMsg = await invoke("greet", { name });
  // }

  async function add(event: Event) {
    event.preventDefault();
    sum = await invoke("add", { a: parseInt(num1), b: parseInt(num2) });
  }
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>



 <form class="row flex " onsubmit={add}>
    <Input id="num1" placeholder="Enter a number..." bind:value={num1} />
    <Input id="num2" placeholder="Enter a number..." bind:value={num2} />
     <!-- <input id="num1" type="number" placeholder="Enter a number..." bind:value={num1} />
    <input id="num2" type="number" placeholder="Enter a number..." bind:value={num2} /> -->
    <Button type="submit">Add</Button>
 </form>

  <p>Sum: {sum}</p>

<!--  -->


  <!-- <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <Button type="submit">Greet</Button>
  </form>
  <p>{greetMsg}</p> -->
</main>

