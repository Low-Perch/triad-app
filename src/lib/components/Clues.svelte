<script lang="ts">
    let { text, revealing = false }: { text: string; revealing?: boolean } = $props()
</script>

<div
    class="relative flex-col mt-12 md:mt-0 w-full uppercase justify-center items-center mx-auto"
    class:clue-reveal={revealing}
>
    <p class="text-2xl md:text-3xl text-center tracking-wider font-semibold text-tone-text">{text}</p>
</div>

<style>
    .clue-reveal {
        animation: clueAppear 0.4s ease-out forwards;
    }

    @keyframes clueAppear {
        0% { opacity: 0; transform: translateY(-10px); }
        100% { opacity: 1; transform: translateY(0); }
    }
</style>
