<script>
  export let headerImageUrl = "/hideoutlogo.png";

  let subscribedSlots = [];
  let selectedSlots = [];

  let timeSlots = [
    { id: 1, time: "11:00–14:00", filled: 0, total: 2 },
    { id: 2, time: "14:00–17:00", filled: 0, total: 2 },
    { id: 3, time: "17:00–21:00", filled: 0, total: 2 }
  ];

  function toggleSlot(slotId) {
    const isSubscribed = subscribedSlots.includes(slotId);
    const isSelected = selectedSlots.includes(slotId);

    if (isSubscribed) {
      // Deselect for unsubscribing
      selectedSlots = isSelected
        ? selectedSlots.filter(id => id !== slotId)
        : [...selectedSlots, slotId];
    } else {
      const slot = timeSlots.find(s => s.id === slotId);
      if (slot.filled >= slot.total || isSelected) return;

      selectedSlots = [...selectedSlots, slotId];
    }
  }

  function confirmChanges() {
    for (const slotId of selectedSlots) {
      const slot = timeSlots.find(s => s.id === slotId);
      const isSubscribed = subscribedSlots.includes(slotId);

      if (isSubscribed) {
        // Unsubscribe
        slot.filled = Math.max(0, slot.filled - 1);
        subscribedSlots = subscribedSlots.filter(id => id !== slotId);
      } else {
        // Subscribe
        if (slot.filled < slot.total) {
          slot.filled++;
          subscribedSlots = [...subscribedSlots, slotId];
        }
      }
    }

    selectedSlots = [];
  }

  function isSlotSelected(slotId) {
    return selectedSlots.includes(slotId);
  }

  function isSlotSubscribed(slotId) {
    return subscribedSlots.includes(slotId);
  }
</script>

<div class="max-w-sm mx-5 mt-21 bg-white rounded-2xl p-6 shadow-lg pb-6">
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
        disabled={
          !isSlotSubscribed(slot.id) && slot.filled >= slot.total
        }
        class={`w-full flex justify-between items-center px-4 py-3 rounded-xl shadow transition
          ${
            isSlotSubscribed(slot.id)
              ? isSlotSelected(slot.id)
                ? 'bg-red-400 text-white'
                : 'bg-green-400 text-white'
              : isSlotSelected(slot.id)
              ? 'bg-orange-500 text-white'
              : slot.filled >= slot.total
              ? 'bg-gray-300 text-gray-500 cursor-not-allowed'
              : 'bg-orange-300 text-black'
          }`}
      >
        <span class="font-medium">{slot.time}</span>
        <span class="text-sm font-semibold">{slot.filled}/{slot.total}</span>
      </button>
    {/each}
  </div>

  <div class="mt-6 p-4 rounded-xl border border-black bg-gray-300 text-sm">
    <p class="font-semibold">SUPERVISOR: JAN DE BOER</p>
    <p>1. BERTJAN</p>
    <p>2. /</p>
  </div>

  <!-- Bevestigen Button -->
  <div class="mt-6 flex justify-center gap-4 flex-wrap">
    <button
      on:click={confirmChanges}
      class={`font-bold py-2 px-4 rounded-xl shadow-lg transition
        ${selectedSlots.length > 0
          ? 'bg-orange-400 text-white hover:bg-orange-500 cursor-pointer'
          : 'bg-gray-300 text-gray-500 cursor-not-allowed'}`}
      disabled={selectedSlots.length === 0}
    >
      BEVESTIGEN
    </button>
  </div>
</div>
