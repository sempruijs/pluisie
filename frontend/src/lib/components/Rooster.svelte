<script>
  export let headerImageUrl = "/hideoutlogo.png";

  let selectedSlots = [];
  let unsubscribingSlots = [];
  let isUnsubscribing = false;

  let timeSlots = [
    { id: 1, time: "11:00–14:00", filled: 0, total: 2 },
    { id: 2, time: "14:00–17:00", filled: 0, total: 2 },
    { id: 3, time: "17:00–21:00", filled: 0, total: 2 }
  ];

  function toggleSlot(slotId) {
    if (isUnsubscribing) {
      const isMarked = unsubscribingSlots.includes(slotId);
      unsubscribingSlots = isMarked
        ? unsubscribingSlots.filter(id => id !== slotId)
        : [...unsubscribingSlots, slotId];
    } else {
      const slot = timeSlots.find(s => s.id === slotId);
      const isSelected = selectedSlots.includes(slotId);
      if (!isSelected && slot.filled >= slot.total) return;

      selectedSlots = isSelected
        ? selectedSlots.filter(id => id !== slotId)
        : [...selectedSlots, slotId];
    }
  }

  function confirmSignup() {
    timeSlots = timeSlots.map(slot =>
      selectedSlots.includes(slot.id) && slot.filled < slot.total
        ? { ...slot, filled: slot.filled + 1 }
        : slot
    );
    selectedSlots = [];
  }

  function startUnsubscribing() {
    isUnsubscribing = true;
    unsubscribingSlots = [];
  }

  function confirmUnsubscribe() {
    timeSlots = timeSlots.map(slot =>
      unsubscribingSlots.includes(slot.id) && slot.filled > 0
        ? { ...slot, filled: slot.filled - 1 }
        : slot
    );
    unsubscribingSlots = [];
    isUnsubscribing = false;
  }

  function cancelUnsubscribing() {
    unsubscribingSlots = [];
    isUnsubscribing = false;
  }
</script>

<div class="max-w-sm mx-auto bg-gray-100 rounded-2xl p-6 shadow-lg pb-6">
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
          (!isUnsubscribing && slot.filled >= slot.total && !selectedSlots.includes(slot.id)) ||
          (isUnsubscribing && slot.filled === 0)
        }
        class={`w-full flex justify-between items-center px-4 py-3 rounded-xl shadow transition
          ${
            isUnsubscribing
              ? unsubscribingSlots.includes(slot.id)
                ? 'bg-red-400 text-white'
                : slot.filled === 0
                  ? 'bg-gray-200 text-gray-400 cursor-not-allowed'
                  : 'bg-orange-300 text-black'
              : selectedSlots.includes(slot.id)
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

  <!-- Action Buttons -->
  <div class="mt-6 flex justify-center gap-4 flex-wrap">
    <!-- AANMELDEN -->
    <button
      on:click={confirmSignup}
      class={`font-bold py-2 px-4 rounded-xl shadow-lg transition 
        ${selectedSlots.length > 0 && !isUnsubscribing
          ? 'bg-orange-400 text-white hover:bg-orange-500 cursor-pointer' 
          : 'bg-gray-300 text-gray-500 cursor-not-allowed'}`}
      disabled={selectedSlots.length === 0 || isUnsubscribing}
    >
      AANMELDEN
    </button>

    <!-- AFMELDEN -->
    <button
      on:click={isUnsubscribing ? confirmUnsubscribe : startUnsubscribing}
      class={`font-bold py-2 px-4 rounded-xl shadow-lg transition 
        ${
          isUnsubscribing
            ? unsubscribingSlots.length > 0
              ? 'bg-red-500 text-white hover:bg-red-600 cursor-pointer'
              : 'bg-gray-300 text-gray-500 cursor-not-allowed'
            : selectedSlots.length > 0 || timeSlots.every(slot => slot.filled === 0)
              ? 'bg-gray-300 text-gray-500 cursor-not-allowed'
              : 'bg-red-400 text-white hover:bg-red-500 cursor-pointer'
        }`}
      disabled={
        selectedSlots.length > 0 ||
        (isUnsubscribing && unsubscribingSlots.length === 0) ||
        (!isUnsubscribing && timeSlots.every(slot => slot.filled === 0))
      }
    >
      {isUnsubscribing ? 'BEVESTIG' : 'AFMELDEN'}
    </button>

    {#if isUnsubscribing}
      <!-- ANNULEREN -->
      <button
        on:click={cancelUnsubscribing}
        class="font-bold py-2 px-4 rounded-xl shadow-lg transition bg-orange-400 text-white hover:bg-orange-500"
      >
        ANNULEREN
      </button>
    {/if}
  </div>
</div>
