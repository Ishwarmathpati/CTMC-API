<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update My Business Units by Key</name>
   <tag></tag>
   <elementGuidId>5f0d2af9-5fb5-417c-826a-109779cbf12f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;version\&quot; : 3,\n  \&quot;actions\&quot; : [ {\n    \&quot;action\&quot; : \&quot;addAddress\&quot;,\n    \&quot;address\&quot; : {\n      \&quot;streetName\&quot; : \&quot;Any Street\&quot;,\n      \&quot;streetNumber\&quot; : \&quot;1337\&quot;,\n      \&quot;postalCode\&quot; : \&quot;11111\&quot;,\n      \&quot;city\&quot; : \&quot;Any City\&quot;,\n      \&quot;country\&quot; : \&quot;US\&quot;\n    }\n  } ]\n}&quot;,
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
      <webElementGuid>a0cf628c-a474-4c4d-8214-d0de95b4563b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c5116a23-eb8f-4237-920f-b8a05dc9b7c6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/me/business-units/key=${business-unit-key}</restUrl>
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
      <id>7cd5f65f-5376-41da-8eff-cd8951fde78d</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>6a3c6c27-4b87-47ca-addb-c52a35ea5e91</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-key</defaultValue>
      <description></description>
      <id>f4335702-10d4-48ac-bff2-43b3a290b2b5</id>
      <masked>false</masked>
      <name>business-unit-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
