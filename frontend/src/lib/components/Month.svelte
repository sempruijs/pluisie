<script lang="ts">
  import { number } from "effect/Equivalence";

    const monthNames: string[] = [
        'Januari', 'Februari', 'Maart', 'April', 'Mei', 'Juni', 'July', 'Augustus', 'September', 'Oktober', 'November', 'December'
    ];

    let currentDate: Date = new Date();

    function prevMonth(): void {
        currentDate.setMonth(currentDate.getMonth() -1);
        currentDate = new Date(currentDate);
    }

    function nextMonth(): void {
        currentDate.setMonth(currentDate.getMonth() + 1);
        currentDate = new Date(currentDate);
    }

    $: currentMonth = monthNames[currentDate.getMonth()];
    $: currentYear = currentDate.getFullYear();
</script>
<div class="flex justify-center">
    <div class="flex items-center justify-between w-70 rounded">
        <span class="font-semibold text-lg">{currentMonth.toUpperCase()} {currentYear}</span>
        <div class="flex space-x-2">
            <button on:click={prevMonth} class="text-gray-400 text-3xl hover:text-black arrowhover">
                &lt;
            </button>
            <button on:click={nextMonth} class="text-gray-400 text-3xl hover:text-black arrowhover">
                &gt;
            </button>
        </div>
    </div>
</div>