<script lang="ts">
    import { getInput } from "../stores/input";

    const input = getInput()
</script>

<div class="flex justify-center items-center my-8 h-40">
    {#each $input.keys as key}
        <div key={key} class="flex m-4 aspect-square border-white border-2 w-12 h-12 justify-center place-items-center">
            <p class="text-white uppercase text-xl">{key}</p>
        </div>
    {/each}
</div>
