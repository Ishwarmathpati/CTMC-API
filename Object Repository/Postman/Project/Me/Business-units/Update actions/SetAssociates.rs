<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetAssociates</name>
   <tag></tag>
   <elementGuidId>f44bf5ed-9649-4c8b-86f3-d4f5434dedee</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${business-unit-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setAssociates\&quot;,\n            \&quot;associates\&quot; : [ {\n              \&quot;customer\&quot; : {\n                \&quot;typeId\&quot; : \&quot;customer\&quot;,\n                \&quot;id\&quot; : \&quot;some-customer-id\&quot;\n              },\n              \&quot;associateRoleAssignments\&quot; : [ {\n                \&quot;associateRole\&quot; : {\n                  \&quot;typeId\&quot; : \&quot;associate-role\&quot;,\n                  \&quot;key\&quot; : \&quot;admin\&quot;\n                },\n                \&quot;inheritance\&quot; : \&quot;Enabled\&quot;\n              } ]\n            }, {\n              \&quot;customer\&quot; : {\n                \&quot;typeId\&quot; : \&quot;customer\&quot;,\n                \&quot;id\&quot; : \&quot;another-customer-id\&quot;\n              },\n              \&quot;associateRoleAssignments\&quot; : [ {\n                \&quot;associateRole\&quot; : {\n                  \&quot;typeId\&quot; : \&quot;associate-role\&quot;,\n                  \&quot;key\&quot; : \&quot;buyer\&quot;\n                }\n              } ]\n            } ]\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>dbb06e55-dde7-4ac5-a208-6747f064ded4</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>03c3abdf-ea60-477e-bf3e-e66bce87f1ec</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/me/business-units/${business-unit-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>d4dbd6e3-9e8b-45e1-b06e-8b400b980f0b</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>671355f6-6b08-45b8-90dd-7a6bba73ab19</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-id</defaultValue>
      <description></description>
      <id>18fd2739-87ce-4d58-b866-628aba7d6073</id>
      <masked>false</masked>
      <name>business-unit-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-version</defaultValue>
      <description></description>
      <id>edebb53c-15b7-4b42-9e2b-957626261be6</id>
      <masked>false</masked>
      <name>business-unit-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
