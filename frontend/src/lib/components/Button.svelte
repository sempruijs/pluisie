<script lang="ts">
  import { createEventDispatcher } from "svelte";
   export let color: string = "green";
  export let padding: string = "default";
  export let width: string = "auto";
  export let className: string = "";
  export let border: string = "border";
  export let hover: string = "";

   const dispatch = createEventDispatcher();
   let color_style = "bg-green-300";
  let padding_style = "p-3 px-7.5";
  let width_style = "w-auto";
  let border_style = "border";
  let hover_style = "hover:bg-green-500";
   if (color === "orange") color_style = "bg-orange-300";
  if (color === "red") color_style = "bg-red-300";
  if (color === "yellow") color_style = "bg-yellow-300";
  if (color === "blue") color_style = "bg-blue-300";
   if (padding === "lg") padding_style = "px-10 py-5";
  if (width === "max") width_style = "w-full";
  if (border === "thick") border_style = "border border-2";
  if (border === "no") border_style = "";

  if (hover === "red") hover_style = "hover:bg-red-500";
  if (hover === "orange") hover_style = "hover:bg-orange-400";
  if (hover === "yellow") hover_style = "hover:bg-yellow-500";
  if (hover === "blue") hover_style = "hover:bg-blue-400";
</script>
 <button
  on:click={(e) => dispatch("click", e)}
  class="{color_style} {width_style} max-h-6 {padding_style} {border_style} shadow-xl shadow-gray-300 rounded-lg flex items-center justify-center text-center {hover_style} hoverbutton {className}"
>
  <slot />
</button>



