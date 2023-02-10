import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
// icons
import { faFolder } from '@fortawesome/free-regular-svg-icons'
import { faAngleRight } from '@fortawesome/free-solid-svg-icons'

export default function (app: any) {
    library.add(faFolder, faAngleRight);
    
    app.component('font-awesome-icon', FontAwesomeIcon)
}