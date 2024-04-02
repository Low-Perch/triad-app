<script lang="ts">
    export let key: string;
    export let width: number = 8;
</script>

<button name={key} class="h-10 w-{width} rounded-sm bg-gray-700 hover:bg-gray-500">
    <span class="text-sm font-semibold uppercase">{key}</span>
</button>
