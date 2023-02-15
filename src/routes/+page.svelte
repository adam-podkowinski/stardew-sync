<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  async function onClick() {
    await invoke("backup_saves", { path: "testPath/" });
  }
</script>

<div class="grid place-items-center px-3 py-6">
  <h1 class="text-4xl font-bold mb-3">Welcome To Stardew-Sync</h1>
  <div>
    <button
      on:click={onClick}
      class="text-2xl bg-blue-300 p-3 rounded text-slate-900">Click me!</button
    >
  </div>
</div>
