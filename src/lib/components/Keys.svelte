<script lang="ts">
    import Key from './Key.svelte'

    const firstRow = 'qwertyuiop'.split('')
    const secondRow = 'asdfghjkl'.split('')
    const lastRow = 'zxcvbnm'.split('')

    let { disabledKeys }: { disabledKeys: string[] } = $props()
</script>

<div class="flex-col justify-center left-1/2 -translate-x-1/2 items-center w-full px-2 absolute bottom-2">
    <div class="flex justify-center gap-x-1.5 items-center">
        {#each firstRow as key (key)}
            <Key key={key} disabled={disabledKeys.includes(key)} />
        {/each}
    </div>

    <div class="flex justify-evenly gap-x-1.5 mx-4 items-center my-1.5">
        {#each secondRow as key (key)}
            <Key key={key} disabled={disabledKeys.includes(key)} />
        {/each}
    </div>

    <div class="flex justify-evenly gap-x-1.5 items-center w-full">
        <Key key="go" width={12} />

        {#each lastRow as key (key)}
            <Key key={key} disabled={disabledKeys.includes(key)} />
        {/each}

        <Key key="del" width={12} />
    </div>
</div>
