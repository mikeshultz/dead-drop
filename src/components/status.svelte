<script lang="ts">
  let { lastUpdated, percent }: { lastUpdated: number; percent: number } =
    $props()

  function humanUnixTime(unix: number): string {
    return unix ? new Date(unix * 1000).toLocaleString() : ""
  }
</script>

<div id="status">
  <div class="updated">{humanUnixTime(lastUpdated)}</div>
  <div class="progress" style:width={`${percent}%`}></div>
</div>

<!-- 
Component that displays last saved time, and shows save progress bar.
//-->
<style type="text/css" lang="css">
  #status,
  #status .updated,
  #status .progress {
    height: 25px;
  }

  #status {
    width: calc(100vw);
    display: block;
    position: fixed;
    top: 0;
    left: 0;
    padding: 0;
    margin: 0;
  }
  #status .updated {
    position: absolute;
    right: 1rem;
    vertical-align: middle;
    line-height: 25px;
    padding-left: 0.5rem;
  }

  #status .progress {
    width: 0%;
    background-color: #eee;
  }
</style>
