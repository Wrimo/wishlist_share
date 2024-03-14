<!-- <script>
    import { onMount } from "svelte";
    import TailwindCss from "./lib/TailwindCSS.svelte";

    let data = "Waiting...";
    onMount(async () => {
        const response = await fetch("http://127.0.0.1:8000/hello/world");
        data = await response.json();
        data = data["hi"];
    });
</script>

<TailwindCss/>

<h1>{data}</h1>
<h1 class="font-sans hover:font-serif">{data}</h1> -->


<script>
    import svelteLogo from './assets/svelte.svg'
    import viteLogo from '/vite.svg'
    import Counter from './lib/Counter.svelte'
    import TailwindCss from './lib/TailwindCSS.svelte';
  </script>
  
  <TailwindCss/>

  <main>
    <h1 class="py-8 px-4 border border-indigo-900 shadow-lg">Hello {name}!</h1>
    <p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
  </main>