<script lang="ts">
  import { number } from "effect/Equivalence";
  import { func } from "effect/FastCheck";
  import { cons } from "effect/List";

    const monthNames: string[] = [
        'Januari', 'Februari', 'Maart', 'April', 'Mei', 'Juni', 'July', 'Augustus', 'September', 'Oktober', 'November', 'December'
    ];

    let currentDate: Date = new Date();
    let fullDays: string[] = [];

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

    $: daysInMonth = [];
    {
        const firstDay = new Date(currentYear, currentMonth, 1);
        const startDay = (firstDay.getDay() + 6) % 7;
        const days = new Date(currentYear, currentMonth + 1, 0).getDate();

        daysInMonth = Array.from({ length: 42 }, (_, i) => {
            const day = i - startDay + 1;
            if (day > 0 && day <= days) {
                return new Date(currentYear, currentDate, day);
            }
            return null;
        });
    }

    function toggleFull(date: Date) {
        const key = date.toISOString().slice(0, 10);
        if (fullDays.includes(key)) {
            fullDays = fullDays.filter(d => d !== key);
        } else {
            fullDays = [...fullDays, key];
        }
    }
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
<div class="grid grid-cols-7 gap-2 mt-4 text-center">
    {#each ['MA', 'DI', 'WO', 'DO', 'VR', 'ZA', 'ZO'] as weekday}
        <div class="font-bold text-sm text-gray-700">{weekday}</div>
    {/each}
</div>