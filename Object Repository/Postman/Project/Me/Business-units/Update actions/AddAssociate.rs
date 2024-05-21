<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddAssociate</name>
   <tag></tag>
   <elementGuidId>aa83984d-d588-423c-a418-f8f89044720c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${business-unit-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;addAssociate\&quot;,\n            \&quot;associate\&quot; : {\n              \&quot;customer\&quot; : {\n                \&quot;typeId\&quot; : \&quot;customer\&quot;,\n                \&quot;id\&quot; : \&quot;some-customer-id\&quot;\n              },\n              \&quot;associateRoleAssignments\&quot; : [ {\n                \&quot;associateRole\&quot; : {\n                  \&quot;typeId\&quot; : \&quot;associate-role\&quot;,\n                  \&quot;key\&quot; : \&quot;admin\&quot;\n                },\n                \&quot;inheritance\&quot; : \&quot;Enabled\&quot;\n              }, {\n                \&quot;associateRole\&quot; : {\n                  \&quot;typeId\&quot; : \&quot;associate-role\&quot;,\n                  \&quot;key\&quot; : \&quot;buyer\&quot;\n                }\n              } ]\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>0d9979ea-377e-4294-90f5-0ab5afe512bf</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>cc93f145-28d8-4996-84ec-cc5dd814aac9</webElementGuid>
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
      <id>1f4bba71-176a-4d6e-a591-cf3e4d6105f9</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>76f3b72a-c2d7-4bd2-9fb7-ce28656287c5</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-id</defaultValue>
      <description></description>
      <id>9bca34d3-73f6-4376-8a80-badd285bf325</id>
      <masked>false</masked>
      <name>business-unit-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-version</defaultValue>
      <description></description>
      <id>ff4ccefb-43cf-422e-a982-0f33f3e54355</id>
      <masked>false</masked>
      <name>business-unit-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
