<script lang="ts">
    import { addKey, removeKey } from '../stores/input'

    export let key: string;
    export let width: number = 8
    export let disabled: boolean = false

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
    disabled={disabled}
    style="width: {width == 10 ? '2rem' : '2.5rem'}"
    on:click|preventDefault|stopPropagation={handleKey}
    class="h-10 rounded-sm bg-gray-700 hover:bg-gray-500 disabled:bg-gray-400 disabled:cursor-auto"
>
    <span class="text-sm font-semibold uppercase w-full">{key}</span>
</button>
