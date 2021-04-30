<template>
  <header class="header">
    <ul class="main-nav">
      <li><router-link to="/">Main</router-link></li>
      <li><router-link to="editor">Editor</router-link></li>
    </ul>
  </header>
</template>
<style lang="scss" scoped>
.header {
  min-height: 80px;
  box-shadow: $gray 0px 10px 10px -10px;
  background-color: white;

  ul.main-nav {
    margin: 0;
    padding: 0;
    list-style: none;

    a {
      color: $dark;
      font-size: 22px;
      padding: 10px 15px;
      text-decoration: none;
      text-transform: uppercase;
      text-align: center;
      display: block;

      &.router-link-active {
        color: #718daa;
      }

      &:hover {
        color: #718daa;
      }
    }
  }
}

@media (min-width: 768px) {
  .header {
    ul.main-nav {
      display: flex;
      justify-content: center;
    }

    display: flex;
    flex-direction: column;
    justify-content: center;
  }
}
</style>
