<script>
  export let headerImageUrl = "/hideoutlogo.png";

  let selectedSlots = [];

  let timeSlots = [
    { id: 1, time: "11:00–14:00", filled: 0, total: 2 },
    { id: 2, time: "14:00–17:00", filled: 0, total: 2 },
    { id: 3, time: "17:00–21:00", filled: 0, total: 2 }
  ];

  function toggleSlot(slotId) {
    const isSelected = selectedSlots.includes(slotId);

    if (isSelected) {
      selectedSlots = selectedSlots.filter(id => id !== slotId);
      timeSlots = timeSlots.map(slot =>
        slot.id === slotId ? { ...slot, filled: slot.filled - 1 } : slot
      );
    } else {
      selectedSlots = [...selectedSlots, slotId];
      timeSlots = timeSlots.map(slot =>
        slot.id === slotId ? { ...slot, filled: slot.filled + 1 } : slot
      );
    }
  }
</script>

<div class="relative max-w-sm mx-auto bg-gray-100 rounded-2xl p-6 shadow-lg pb-20">
  <!-- Header -->
  <div class="mb-6">
    <img src={headerImageUrl} alt="Header" class="w-full h-auto object-contain" />
  </div>

  <h2 class="text-center text-lg font-semibold tracking-wide mb-4">VRIJDAG 20 JUNI</h2>

  <!-- Time Slots -->
  <div class="space-y-4">
    {#each timeSlots as slot}
      <button
        on:click={() => toggleSlot(slot.id)}
        class={`w-full flex justify-between items-center px-4 py-3 rounded-xl shadow transition
          ${
            selectedSlots.includes(slot.id)
              ? 'bg-orange-500 text-white'
              : 'bg-orange-300 text-black'
          }`}
      >
        <span class="font-medium">{slot.time}</span>
        <span class="text-sm font-semibold">{slot.filled}/{slot.total}</span>
      </button>
    {/each}
  </div>

  <!-- Supervisor -->
  <div class="mt-6 p-4 rounded-xl border border-black bg-gray-300 text-sm">
    <p class="font-semibold">SUPERVISOR: JAN DE BOER</p>
    <p>1. BERTJAN</p>
    <p>2. /</p>
  </div>

  <!-- Fixed Sign Up Button -->
  <button
    class={`absolute bottom-4 right-4 font-bold py-2 px-4 rounded-xl shadow-lg transition 
      ${selectedSlots.length > 0 
        ? 'bg-orange-400 text-white hover:bg-orange-500 cursor-pointer' 
        : 'bg-gray-300 text-gray-500 cursor-not-allowed'}`}
    disabled={selectedSlots.length === 0}
  >
    AANMELDEN
  </button>
</div>
