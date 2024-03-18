<script>
  import { onMount } from "svelte";
  import TailwindCss from "./lib/TailwindCSS.svelte";

  let data = "Waiting...";
  onMount(async () => {
      const response = await fetch("http://127.0.0.1:8000/family");
      data = await response.json();
      console.log(data);
});
</script>

<TailwindCss />

<h1 class="font-sans hover:font-serif fixed top-1 left">Hello!</h1>
{#each data as person, i}  
  <p class="text-xl">{person["first_name"] + " " + person["last_name"]}</p>
{/each}