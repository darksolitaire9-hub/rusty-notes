<script lang="ts">
  let note = "# Rusty Notes\n\nWrite something here...";
  
  function save() {
    console.log("Saving note:", note);
    alert("Saved! (Check console)");
  }
</script>

<div class="app">
  <h1>🦀 Rusty Notes</h1>
  <textarea bind:value={note}></textarea>
  <button on:click={save}>Save Note</button>
</div>

<style>
  :global(body) { margin: 0; font-family: sans-serif; background: #222; color: #fff; }
  .app { display: flex; flex-direction: column; height: 100vh; padding: 20px; box-sizing: border-box; }
  textarea { flex: 1; background: #333; color: #fff; border: 1px solid #444; padding: 10px; font-size: 16px; margin-bottom: 10px; resize: none; }
  button { padding: 10px 20px; background: #e94e18; color: white; border: none; cursor: pointer; font-weight: bold; }
  button:hover { background: #ff5e00; }
</style>
