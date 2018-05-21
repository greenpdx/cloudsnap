import Vue from 'vue'
import Router from 'vue-router'
import Home from '../pages/home/Home'
import Wiki from '../pages/wiki/Wiki'
import Explore from '../pages/explore/Explore'
import Theme from '../pages/theme/Theme'
import Post from '../pages/new/Post'
import Create from '../pages/new/Create'
import Community from '../pages/community/Community'
import Access from '../pages/user/Access'
import SignUp from '../pages/user/SignUp'
import Center from '../pages/user/Center'
import More from '../pages/more/More'
import NotFound from '../pages/notfound/NotFound'
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