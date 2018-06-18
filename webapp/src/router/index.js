import Vue from 'vue'
import Router from 'vue-router'
import Home from '../views/home/Home'
import Wiki from '../views/wiki/Wiki'
import Explore from '../views/explore/Explore'
import Theme from '../views/theme/Theme'
import Post from '../views/new/Post'
import Create from '../views/new/Create'
import Community from '../views/community/Community'
import Access from '../views/user/Access'
import SignUp from '../views/user/SignUp'
import Center from '../views/user/Center'
import More from '../views/more/More'
import NotFound from '../views/notfound/NotFound'
Vue.use(Router)
export default new Router({
  mode: 'history',
  linkActiveClass: 'is-active',
  routes: [
    { path: '/', name: 'home', component: Home },
    { path: '/a/wiki', name: 'wiki', component: Wiki },
    { path: '/a/explore', name: 'explore', component: Explore },
    { path: '/a/:community/theme/:id', name: 'theme', component: Theme },
    { path: '/a/post', name: 'post', component: Post },
    { path: '/a/create', name: 'create', component: Create },
    { path: '/a/access', name: 'access', component: Access },
    { path: '/a/signup', name: 'signup', component: SignUp },
    { path: '/a/user/:id', name: 'user', component: Center },
    { path: '/a/community/:name', name: 'community', component: Community },
    { path: '/a/more', name: 'more', component: More },
    { path: '*', name: 'notfound', component: NotFound }
  ]
})