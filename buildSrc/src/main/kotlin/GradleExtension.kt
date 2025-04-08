import org.gradle.api.Action
import org.gradle.api.Task
import org.gradle.api.provider.Provider
import org.gradle.api.tasks.TaskContainer
import org.gradle.api.tasks.TaskProvider
import org.gradle.kotlin.dsl.configure

fun <T : Task> TaskProvider<T>.orProvide(other: TaskProvider<T>): TaskProvider<T> =
    object : TaskProvider<T>, Provider<T> by orElse(other) {
        override fun configure(action: Action<in T>) {
            val instance = this.get()
            instance.configure<Task>(configuration = {
                action.execute(instance)
            })
        }

        override fun getName(): String {
            return get().name
        }
    }

inline fun <reified T : Task> TaskContainer.registerSafe(
    name: String,
    taskClass: Class<T>,
    crossinline register: T.() -> Unit,
): T = (findByName(name) as T?) ?: register(name, taskClass) { register(this) }.get()

fun TaskContainer.registerSafe(name: String, register: Task.() -> Unit): Task =
    registerSafe(name, Task::class.java, register)

val TaskContainer.generateHeaders: CbindGenerate get() = withType(CbindGenerate::class.java).first()