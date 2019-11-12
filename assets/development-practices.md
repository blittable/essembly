# Essembly Development Practices

This is the draft version of development practices based on Agile.
There are 3 major sections to focus

* ## Development Process
    * ### Development
        * #### Transparency
            Be able to check what happen about the data and process.

            * **Process Timeline**  
                To visualize set of processes and comsuming time as timeline.
                
            * **Module Interaction**  
                To visualize interaction of modules in the process into graph form.
            
            * **Data Transformation**  
                See what happen about the data and how they transform.
        
        * #### Debugging
            * **Process Reproduction**  
                Be able to reproduce any process again with the same environment and the same data that was snapshot as binary file.

        * #### Framework Agreement
            * **Payload/Input**  
                Do the specification the standard form of argument name, type, description. for example,

                `BasePayload.cs`
                ```c#
                public class SortableArgument
                {
                    public static string ASC = "asc";
                    public static string DESC = "desc";
                    public string by { get; set; }
                    public string order { get; set; }
                }

                public abstract class BasePayload
                {
                    public bool Visible { get; set; }
                    public abstract JObject Argument { get; set; }
                    public abstract JObject Sample { get; }
                    public abstract Type ArgumentType { get; }
                }
                public interface ITransformablePayload
                {
                    IEnumerable<T> Transform<T>(IEnumerable<T> objects);
                }
                public static class ITransformPayloadExtension
                {
                    public static IEnumerable<T> OnTransform<T>(this ITransformablePayload payload, IEnumerable<T> objects)
                    {
                        return payload.Transform(objects);
                    }
                }

                public class GuidArgument
                {
                    public string guid { get; set; }
                }

                public interface IParameterablePayload
                {
                    T GetParameter<T>();
                }
                ```

                `SortableParameter.cs`
                ```c#
                public class SortableParameter : BasePayload, IParameterablePayload
                {
                    private SortableArgument _argument;

                    public override JObject Argument
                    {
                        get => JObject.FromObject(new SortableArgument() { by = "<string>", order = "<string>(asc/desc)" });
                        set => _argument = value.ToObject<SortableArgument>();
                    }

                    public override JObject Sample
                    {
                        get => JObject.FromObject(new SortableArgument()
                        {
                            by = "name",
                            order = "asc"
                        });
                    }

                    public override Type ArgumentType => typeof(SortableArgument);

                    public T GetParameter<T>()
                    {
                        return (T)((object)_argument);
                    }
                }
                ```

            * **Response/Output**  
                Do the specification the standard output form.

        * #### Logging
            * **Activity Archive**  
                To keep the binary file of an action for debugging.
        
        * #### Unit Testing
    
    * ### Milestone
        Group tasks togeter for achieve the feature with a number of user story that selected from **Product Backlog**

        * #### Task
            Describe the actual work that developer need to do.

    * ### Deployment
        * #### Release Cycle
            * **Versioning**  
                Describe the version number by **Major**.**Minor**.**Changes** of a module. What the rule of increasing each number.
        
        * #### Server
            * **On-Permise**  
                For the service need to be the internal hosting. Any secret data. Transactions, Keys...
            
            * **Cloud**  
                To support 24/7 high avalibility services.
        
        * #### Product Distribution
            * **Service Customization**  
                Every module can breakdown can setup as a product to be appropriate with the business.
    
    * ### Integration
        * #### Automation
            * **CI/CD**  
                To make product ready for UAT and control how to promote to the production.

            * **Compatibility Check**  
                Do checking for compatible in each version of the module.

        * #### IoT Devices
        * #### 3rd Party Service

* ## Team Collaboration
    * ### Stakeholder & Product Owner
        * #### User Story
            * **Product Backlog**  
                Describe for any idea, task, note for transform into more clarify tasks.

    * ### Product Owner & Developer
        * #### Tasks
            PO/PM and Developers working together to translate the User Story into Tasks. The task is solid form, it describes clearly what actual work that the developer needs to do.
        
        * #### Technical Discussion
            Sometime we got the technical problems that maybe change the project plan. We need the discuss with PO/PM to find the good solution.

    * ### Developer Team
        * #### Code Review
            * **Code Smell**  
                To check healty of the code.
                http://mikamantyla.eu/BadCodeSmellsTaxonomy.html
            
            * **Code Quality**

        * #### Coding Standard
            Specificy the naming convenstion of paramter, method, namespace, package, module, etc...

        * #### Source Control  
            * **Pull Request/Merge Request**  
                Developer need to create the new branch corresponding to the task # and tiny description. The merge process will accept by lead developer.

    * ### Follow-up
        * #### Relationship of User Story x Time 
            * **Fixed scope**  
                To determine when the specific features will be ready. Eailer before or not too late before.
            * **Fixed time**
                To determine what the set of features that we can release with specific period.
    
    * ### Storage
        * **Backup**
            To store changes of database, server, versions, etc...
