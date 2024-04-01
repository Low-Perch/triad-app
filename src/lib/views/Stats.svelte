<script lang="ts">
    import { closeModal } from '../stores/modal'
</script>

<div class="flex flex-col w-full h-full text-white p-5 py-3 justify-center gap-3 items-center">
    <h2 class="font-semibold text-lg text-center">Stats</h2>

    <p class="text-md text-center">
        You solved this puzzle in 23s without using any lifelines. Nice work!
    </p>

    <div class="flex justify-between gap-x-12 gap-y-4 flex-wrap w-full">
        <div class="flex-col">
            <p class="text-md font-semibold text-center">38</p>
            <p class="text-md">Played</p>
        </div>

        <div class="flex-col">
            <p class="text-md font-semibold text-center">38</p>
            <p class="text-md">Solved</p>
        </div>

        <div class="flex-col">
            <p class="text-md font-semibold text-center">100%</p>
            <p class="text-md">Solve Rate</p>
        </div>

        <div class="flex-col">
            <p class="text-lg font-semibold text-center">8</p>
            <p class="text-md">Solve Streak</p>
        </div>

        <div class="flex-col">
            <p class="text-lg font-semibold text-center">2s</p>
            <p class="text-md">Best Time</p>
        </div>
    </div>

    <div class="flex-col justify-between w-full gap-y-16">
        <p class="text-center font-semibold text-sm">Solve Times</p>

        <div class="flex justify-between w-full">
            <div class="flex-col items-center">
                <p class="text-lg font-semibold text-center">1</p>
                <p class="text-md text-center">{"<10s"}</p>
            </div>

            <div class="flex-col items-center">
                <p class="text-lg font-semibold text-center">1</p>
                <p class="text-md text-center">{"<30s"}</p>
            </div>

            <div class="flex-col items-center">
                <p class="text-lg font-semibold text-center">1</p>
                <p class="text-md text-center">{"60s"}</p>
            </div>


            <div class="flex-col items-center">
                <p class="text-lg font-semibold text-center">3</p>
                <p class="text-md text-center">{"<5m"}</p>
            </div>
        </div>
    </div>

    <div class="flex w-full justify-between h-full items-center">
        <button 
            on:click={closeModal}
            class="items-center inline-flex justify-center w-32 h-10 border-2 rounded-3xl border-white">
            <span class="text-md">View Puzzle</span>
        </button>

        <button 
            on:click={closeModal}
            class="items-center inline-flex bg-[#26b726] justify-center w-24 h-10 border-2 rounded-3xl border-[#26b726]">
            <span class="text-md text-white">Share</span>
        </button>
    </div>
</div>
