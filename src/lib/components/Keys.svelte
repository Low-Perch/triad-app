<script lang="ts">
    import Key from './Key.svelte'

    const firstRow = 'qwertyuiop'.split('')
    const secondRow = 'asdfghjkl'.split('')
    const lastRow = 'zxcvbnm'.split('')

    let { disabledKeys }: { disabledKeys: string[] } = $props()
</script>

<div class="keys-container">
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

<style>
    .keys-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        max-width: 400px;
        margin: 0 auto;
        padding: 0 0.5rem;
        position: absolute;
        bottom: 0.5rem;
        left: 50%;
        transform: translateX(-50%);
    }
</style>
