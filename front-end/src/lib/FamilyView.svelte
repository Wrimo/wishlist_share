<script>
    import { onMount } from "svelte";
    import TailwindCss from "./TailwindCSS.svelte";

    export let person_id;
    let data = [];

    onMount(async () => {
        console.log(person_id);
        const response = await fetch(
            `http://127.0.0.1:8000/family/${person_id}`,
        );
        data = await response.json();
        console.log(data);
    });
</script>

<TailwindCss />

<div class="bg-slate-100 rounded-xl scale-150">
    <h1 class="p-2 text-black text-5xl">Your Family</h1>
    <div class="container m-auto grid grid-cols-12 text-white rounded-xl gap-1">
        {#each data as person, i}
            <div class="col-span-12 p-2 rounded-xl">
                <button class="w-96 bg-slate-300 text-left text-black"
                    >{person["first_name"] + " " + person["last_name"]}</button
                >
            </div>
        {/each}
    </div>
</div>
