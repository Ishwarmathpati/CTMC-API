<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetCustomField</name>
   <tag></tag>
   <elementGuidId>ada5a979-418c-4cf9-8cea-9ba628d96b23</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${quote-request-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setCustomField\&quot;,\n            \&quot;name\&quot; : \&quot;ExampleStringTypeField\&quot;,\n            \&quot;value\&quot; : \&quot;TextString\&quot;\n          }\n    ]\n}&quot;,
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
      <webElementGuid>87d690fa-f88d-40bf-ba54-03d67da4a5ac</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>90ccf8b9-d1e6-4e3d-81b4-888d1d0df064</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/quote-requests/${quote-request-id}</restUrl>
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
      <id>ecfcba2f-173b-4bee-957d-197947d45557</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>4b01a451-df75-4779-b1ed-81f45f2a0db5</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.quote-request-id</defaultValue>
      <description></description>
      <id>bf0ead14-08dd-4e14-b386-407aac4b9d82</id>
      <masked>false</masked>
      <name>quote-request-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.quote-request-version</defaultValue>
      <description></description>
      <id>1f74994c-b794-4134-b161-92eff188eb13</id>
      <masked>false</masked>
      <name>quote-request-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
