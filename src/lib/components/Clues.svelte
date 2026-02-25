<script lang="ts">
    let { text }: { text: string } = $props()
</script>

<div class="relative flex-col mt-12 w-11/12 uppercase justify-center items-center mx-auto">
    <p class="text-3xl text-center tracking-wider font-semibold text-tone-text">{text}</p>
</div>
