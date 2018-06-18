<template>
  <div id="app">
    <div id="body">
      <router-view></router-view>
    </div>
  </div>
</template>

<script>
export default {
  name: 'app'
}
</script>

<style>
:root {
    --purple: #99177d;
}
* {
    padding:0px; margin:0px;
    box-sizing: border-box;
}
a {
  text-decoration: none;
  color: inherit;
}
a:hover { color: var(--purple); }

ul, ol, li {
    list-style: none;
}
#body {
  background-color: #faf5f5;
  min-height: 100vh;
}
</style>
