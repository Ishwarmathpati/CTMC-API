<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetVariantSelection</name>
   <tag></tag>
   <elementGuidId>c85bc981-4b4d-4e3a-9ac1-b3793649d1b1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${product-selection-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setVariantSelection\&quot;,\n            \&quot;product\&quot; : {\n              \&quot;typeId\&quot; : \&quot;product\&quot;,\n              \&quot;key\&quot; : \&quot;millennium-falcon\&quot;\n            },\n            \&quot;variantSelection\&quot; : {\n              \&quot;type\&quot; : \&quot;includeOnly\&quot;,\n              \&quot;skus\&quot; : [ \&quot;M0E20000000EH3V\&quot; ]\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>1ebe9f9c-7115-4c99-8098-1fa8952dd7cb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ce7cfc13-e266-471e-bd48-acea220634f6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/product-selections/${product-selection-id}</restUrl>
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
      <id>6d6542da-0a61-42f4-9721-04b38c9190e7</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>43f19564-f101-40da-b813-c500b14bf8a5</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.product-selection-id</defaultValue>
      <description></description>
      <id>30411299-5d1e-438e-bf38-3b1d52a4cd4c</id>
      <masked>false</masked>
      <name>product-selection-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.product-selection-version</defaultValue>
      <description></description>
      <id>3078d138-21da-4b6b-8bd1-04479f1e71a3</id>
      <masked>false</masked>
      <name>product-selection-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
