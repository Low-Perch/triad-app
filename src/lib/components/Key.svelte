<script lang="ts">
    import { addKey, removeKey } from '../stores/input'

    export let key: string;
    export let width: number = 8;

    function handleKey(e: Event)  {
        const button = e.currentTarget as HTMLButtonElement
        const key = button.name.toUpperCase()

        if (key == "GO") {
            // TODO
        } else if (key == "DEL") {
            removeKey()
        } else {
            addKey(key)
        }
    }
</script>

<button 
    name={key}
    on:click|preventDefault|stopPropagation={handleKey}
    class="h-10 w-{width} rounded-sm bg-gray-700 hover:bg-gray-500"
>
    <span class="text-sm font-semibold uppercase">{key}</span>
</button>
