<script lang="ts">
    let { text }: { text: string } = $props()
</script>

<div class="relative flex-col mt-12 w-full uppercase justify-center items-center mx-auto">
    <p class="text-2xl text-center tracking-wider font-semibold text-tone-text">{text}</p>
</div>
