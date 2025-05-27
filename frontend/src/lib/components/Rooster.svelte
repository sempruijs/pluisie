<script>
  export let headerImageUrl = "/hideoutlogo.png";

  let selectedSlot = null;

  const timeSlots = [
    { id: 1, time: "11:00–14:00", filled: 1, total: 2 },
    { id: 2, time: "14:00–17:00", filled: 1, total: 2 },
    { id: 3, time: "17:00–21:00", filled: 1, total: 2 }
  ];

  function selectSlot(slotId) {
    selectedSlot = slotId;
  }
</script>

<div class="max-w-sm mx-auto bg-gray-100 rounded-2xl p-6 shadow-lg">
  <!-- Header -->
  <div class="mb-6">
    <img src={headerImageUrl} alt="Header" class="w-full h-auto object-contain" />
  </div>

  <!-- Date -->
  <h2 class="text-center text-lg font-semibold tracking-wide mb-4">VRIJDAG 20 JUNI</h2>

  <!-- Time Slots -->
  <div class="space-y-4">
    {#each timeSlots as slot}
      <button
        on:click={() => selectSlot(slot.id)}
        class={`w-full flex justify-between items-center px-4 py-3 rounded-xl shadow transition
          ${
            selectedSlot === slot.id
              ? 'bg-orange-500 text-white'
              : 'bg-orange-300 text-black'
          }`}
      >
        <span class="font-medium">{slot.time}</span>
        <span class="text-sm font-semibold">{slot.filled}/{slot.total}</span>
      </button>
    {/each}
  </div>

  <!-- Supervisor Section -->
  <div class="mt-6 p-4 rounded-xl border border-black bg-gray-300 text-sm">
    <p class="font-semibold">SUPERVISOR: JAN DE BOER</p>
    <p>1. BERTJAN</p>
    <p>2. /</p>
  </div>

  <!-- Sign Up Button -->
  <button class="mt-6 w-full bg-orange-400 text-white font-bold py-2 rounded-xl hover:bg-orange-500 transition">
    AANMELDEN
  </button>
</div>
