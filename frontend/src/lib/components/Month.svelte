<script lang="ts">
    const monthNames: string[] = [
        'Januari', 'Februari', 'Maart', 'April', 'Mei', 'Juni',
        'July', 'Augustus', 'September', 'Oktober', 'November', 'December'
    ];

    let currentDate: Date = new Date();
    let fullDays: string[] = [];

    function prevMonth(): void {
        currentDate.setMonth(currentDate.getMonth() - 1);
        currentDate = new Date(currentDate);
    }

    function nextMonth(): void {
        currentDate.setMonth(currentDate.getMonth() + 1);
        currentDate = new Date(currentDate);
    }

    $: currentMonth = monthNames[currentDate.getMonth()];
    $: currentYear = currentDate.getFullYear();

    $: daysInMonth = (() => {
        const firstDay = new Date(currentYear, currentDate.getMonth(), 1);
        const startDay = (firstDay.getDay() + 6) % 7; // Monday = 0
        const numDays = new Date(currentYear, currentDate.getMonth() + 1, 0).getDate();

        return Array.from({ length: 42 }, (_, i) => {
            const day = i - startDay + 1;
            if (day > 0 && day <= numDays) {
                return new Date(currentYear, currentDate.getMonth(), day);
            }
            return null;
        });
    })();

    function toggleFull(date: Date) {
        const key = date.toISOString().slice(0, 10);
        if (fullDays.includes(key)) {
            fullDays = fullDays.filter(d => d !== key);
        } else {
            fullDays = [...fullDays, key];
        }

        fetch("/api/save-shift", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ date: key, full: !fullDays.includes(key) }),
        });
    }

    function isFull(date: Date): boolean {
        const key = date.toISOString().slice(0, 10);
        return fullDays.includes(key);
    }
</script>

<div class="flex justify-center mb-4">
    <div class="flex items-center justify-between w-72 rounded">
        <span class="font-semibold text-lg">{currentMonth.toUpperCase()} {currentYear}</span>
        <div class="flex space-x-2">
            <button on:click={prevMonth} class="text-gray-400 text-3xl arrowhover hover:text-black">&lt;</button>
            <button on:click={nextMonth} class="text-gray-400 text-3xl arrowhover hover:text-black">&gt;</button>
        </div>
    </div>
</div>

<div class="grid grid-cols-7 gap-2 text-center text-sm">
    {#each ['MA', 'DI', 'WO', 'DO', 'VR', 'ZA', 'ZO'] as weekday}
        <div class="font-bold text-gray-700">{weekday}</div>
    {/each}

    {#each daysInMonth as date}
    <div>
    {#if date}
        <script>
            const day = date.getDate();
        </script>
        <button 
            on:click={() => toggleFull(date)}
            class="w-8 h-8 rounded-full transition-all duration-150 font-medium 
                {isFull(date)
                    ? 'bg-red-300 text-white'
                    : [1, 2, 3, 4, 5].includes(date.getDay())
                        ? 'bg-green-200 arrowhover calendarhover hover:bg-orange-300'
                        : 'bg-gray-300'}"
        >
            {date.getDate()}
        </button>
    {:else}
        <div></div>
    {/if}
    </div>
{/each}

</div>
