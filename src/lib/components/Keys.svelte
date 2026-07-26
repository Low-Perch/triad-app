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

    <div class="flex justify-center gap-x-1.5 items-center my-1.5">
        {#each secondRow as key (key)}
            <Key key={key} disabled={disabledKeys.includes(key)} />
        {/each}
    </div>

    <div class="flex justify-center gap-x-1.5 items-center">
        <Key key="go" width={12} />

        {#each lastRow as key (key)}
            <Key key={key} disabled={disabledKeys.includes(key)} />
        {/each}

        <Key key="del" width={12} />
    </div>
</div>

<style>
    /* In flow: margin-top auto pins it to the bottom of the game area on
       small screens; wide screens join it to the centered stack instead */
    .keys-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        max-width: 400px;
        margin: auto auto 0.5rem;
        padding: 0 0.5rem;
    }

    @media (min-width: 768px) {
        .keys-container {
            max-width: 520px;
            margin: 2.5rem auto 0;
        }
    }
</style>
