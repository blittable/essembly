# Essembly Development Practices

This is the draft version of development practices based on Agile.

There are 3 major sections: 

* ## Development Process
    * ### Development

        * #### Transparency

            Be able to verify what happens with the process.

            * **Process Timeline**  
                To visualize set of processes and consuming time as timeline.
                
            * **Module Interaction**  
                To visualize interaction of modules in the process as a graph.
            
            * **Data Transformation**  
                See what happens with data in the system and how it is transformed.
        
        * #### Debugging

            * **Process Reproduction**  
                Be able to reproduce any process again with the same environment and the same data along with capturing or rebuilding the related code at the exact version level.

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
        Group tasks together for delivering the feature with a number of user stories taken from the  the **Product Backlog**

        * #### Task
            Describe the actual work that developer needs to do.

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
                Detail any idea, task, or other notes for transforming into clear tasks.

    * ### Product Owner & Developer
        * #### Tasks
            PO/PM and Developers work together to translate the User Story into Tasks. The task is concrete, it describes clearly what actual work a developer needs to do.
        
        * #### Technical Discussion
            Sometime we got have technical problems that can alter the project plan. This needs to be discussed with the PO/PM to find a solution.

    * ### Developer Team
        * #### Code Review
            * **Code Smells**  
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
                
* ## Post-Process & BAU 
    
    * ### Storage
        * **Backup**
            To store changes of database, server, versions, etc...
