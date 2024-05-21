<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ChangeFieldDefinitionOrder</name>
   <tag></tag>
   <elementGuidId>99c4a3bd-f2b8-4f4a-89e0-2442a3752c7b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${type-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;changeFieldDefinitionOrder\&quot;,\n            \&quot;fieldNames\&quot; : [ \&quot;CustomENumField\&quot;, \&quot;CustomLeNumField\&quot;, \&quot;CustomTextField\&quot; ]\n          }\n    ]\n}&quot;,
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
      <webElementGuid>6c7c3b8f-3ff9-4d7a-b493-e0351b18b107</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>300f26f0-8218-40dd-8396-ed2396b668d3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/types/${type-id}</restUrl>
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
      <id>96e7c589-59a6-4a51-b53c-6ef4d112f701</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>07e47364-c3ba-48dc-9e8f-d689b1cbe2f4</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.type-id</defaultValue>
      <description></description>
      <id>d4bb91f0-caaa-43b4-803a-861cb1a8fd92</id>
      <masked>false</masked>
      <name>type-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.type-version</defaultValue>
      <description></description>
      <id>a648d25a-d8bf-4a30-89fc-0a942a3c6f92</id>
      <masked>false</masked>
      <name>type-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
