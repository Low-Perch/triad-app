<script lang="ts">
    import Key from './Key.svelte'

    const firstRow = 'qwertyuiop'.split('')
    const secondRow = 'asdfghjkl'.split('')
    const lastRow = 'zxcvbnm'.split('')

    export let disabledKeys: string[]
</script>

<div class="flex-col justify-center left-1/2 -translate-x-1/2 items-center w-full px-4 absolute bottom-3">
    <div class="flex justify-center gap-x-2 items-center">
        {#each firstRow as key (key)}
            <Key key={key} disabled={disabledKeys.includes(key)} />
        {/each}
    </div>

    <div class="flex justify-evenly gap-x-2 mx-4 items-center my-2">
        {#each secondRow as key (key)}
            <Key key={key} disabled={disabledKeys.includes(key)} />
        {/each}
    </div>

    <div class="flex justify-evenly gap-x-2 items-center w-full">
        <Key key="go" width={12} />

        {#each lastRow as key (key)}
            <Key key={key} disabled={disabledKeys.includes(key)} />
        {/each}

        <Key key="del" width={12} />
    </div>
</div>
