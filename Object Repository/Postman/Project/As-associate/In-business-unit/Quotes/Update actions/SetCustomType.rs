<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetCustomType</name>
   <tag></tag>
   <elementGuidId>1e543482-08ce-421d-a8a5-a0caef688dd3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${quote-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setCustomType\&quot;,\n            \&quot;type\&quot; : {\n              \&quot;id\&quot; : \&quot;${type-id}\&quot;,\n              \&quot;typeId\&quot; : \&quot;type\&quot;\n            },\n            \&quot;fields\&quot; : {\n              \&quot;exampleStringTypeField\&quot; : \&quot;TextString\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>0931f9ee-ad33-497c-9b99-8d32326e449d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>3ed89cdb-a327-47bb-a67d-707ddb722b30</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/as-associate/${associate-id}/in-business-unit/key=${business-unit-key}/quotes/${quote-id}</restUrl>
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
      <id>5e976bc4-34fc-479d-9b4d-8aa8c1be2e3d</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>7ae54cdb-7e57-4d08-81d1-5bcca4b65a11</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.associate-id</defaultValue>
      <description></description>
      <id>b52bdece-0783-4cdf-bda6-a56b0a104de2</id>
      <masked>false</masked>
      <name>associate-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-key</defaultValue>
      <description></description>
      <id>56706c09-deb8-4729-ac25-152fcb69226d</id>
      <masked>false</masked>
      <name>business-unit-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.quote-id</defaultValue>
      <description></description>
      <id>229cb63b-32f2-40e0-b099-c3ecb1cc03c6</id>
      <masked>false</masked>
      <name>quote-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.quote-version</defaultValue>
      <description></description>
      <id>a0dc0a5c-8b9d-4503-98b8-af1751459e55</id>
      <masked>false</masked>
      <name>quote-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.type-id</defaultValue>
      <description></description>
      <id>9df91bfa-a8b0-451c-bc44-78b654083533</id>
      <masked>false</masked>
      <name>type-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
